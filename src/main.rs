extern crate liebert_mpx as liebert;
use rumqttc::{MqttOptions, AsyncClient, QoS};
use std::time::{Duration, Instant};
use std::thread::sleep;
use std::pin::Pin;
use std::future::Future;
use tokio::sync::mpsc;
use ini::Ini;

mod mqttify;
use crate::mqttify::ToMQTT;

#[derive(Copy,Clone,PartialEq)]
enum TaskPriority {
    HIGH,
    LOW,
}

#[derive(Clone,PartialEq,Debug)]
pub struct MQTTMsg {
    topic: String,
    payload: String,
    retained: bool,
}
pub type MQTTMsgList = Vec<MQTTMsg>;

enum Cache {
    MQTTMsgList(MQTTMsgList),
    EventList(liebert::EventList),
    None(()),
}

impl Cache {
    fn get_modified(self: &Self, new: &MQTTMsgList) -> MQTTMsgList {
        let mut result : MQTTMsgList = Vec::new();

        match self {
            Cache::MQTTMsgList(old) => {
                for n in new {
                    for o in old {
                        if n.topic == o.topic {
                            if n.payload != o.payload {
                                result.push(n.clone());
                            }
                            break;
                        }
                    }
                }
            },
            _ => {
                /* no cached data => everything is new */
                result.append(&mut new.clone());
            }
        }
        
        result
    }

    fn is_none(self: &Self) -> bool {
        match self {
            Cache::None(()) => true,
            _ => false,
        }
    }
}

struct Task {
    timestamp: Instant,
    timeout: Duration,
    priority: TaskPriority,
    function: fn(&'_ mut Task) -> Pin<Box<dyn Future<Output = Result<MQTTMsgList, liebert::MPXError>> + Send + '_>>,
    mpx: std::sync::Arc<liebert::MPX>,
    pdu: u8,
    branch: u8,
    receptacle: u8,
    cache: Cache,
}

impl Task {
    fn update_timestamp(self: &mut Self) -> () {
        self.timestamp = Instant::now();
    }

    async fn run(self: &mut Self) -> Result<MQTTMsgList, liebert::MPXError> {
        let result = (self.function)(self).await;
        self.update_timestamp();
        result
    }

    fn timed_out(self: &mut Self) -> bool {
        self.timestamp.elapsed() > self.timeout
    }

    fn reschedule_in(self: &mut Self, seconds: u8) -> () {
        let offset = Duration::from_secs(seconds.into());
        self.timestamp = Instant::now().checked_sub(self.timeout).expect("Time out of bounds").checked_add(offset).expect("Time out of bounds");
    }
}

type TaskList = Vec<Task>;

trait TaskListFunctions {
    fn append_systemd_watchdog(&mut self, mpx: std::sync::Arc<liebert::MPX>) -> ();
    fn get_oldest(&mut self) -> Option<&mut Task>;
    fn contains(self: &Self, pdu: u8, branch: u8, receptacle: u8) -> bool;
    fn reschedule_in(self: &mut Self, pdu: u8, branch: u8, receptacle: u8, seconds: u8) -> ();
}

impl TaskListFunctions for TaskList {
    fn get_oldest(self: &mut Self) -> Option<&mut Task> {
        self.iter_mut().max_by(|a, b| a.timestamp.elapsed().partial_cmp(&b.timestamp.elapsed()).unwrap_or(std::cmp::Ordering::Less))
    }

    fn contains(self: &Self, pdu: u8, branch: u8, receptacle: u8) -> bool {
        for task in self {
            if task.pdu == pdu && task.branch == branch && task.receptacle == receptacle {
                return true;
            }
        }

        false
    }

    fn reschedule_in(self: &mut Self, pdu: u8, branch: u8, receptacle: u8, seconds: u8) -> () {
        for task in self {
            if task.pdu == pdu && task.branch == branch && task.receptacle == receptacle {
                task.reschedule_in(seconds);
            }
        }
    }

    fn append_systemd_watchdog(self: &mut Self, mpx: std::sync::Arc<liebert::MPX>) -> () {
        self.push(Task {
            timestamp: Instant::now(),
            timeout: Duration::from_secs(30),
            priority: TaskPriority::HIGH,
            function: |t| Box::pin(systemd_watchdog_update(t)),
            mpx: mpx,
            pdu: 0,
            branch: 0,
            receptacle: 0,
            cache: Cache::None(()),
        });
    }
}

async fn read_receptacle(task: &mut Task) -> Result<MQTTMsgList, liebert::MPXError> {
    let info = task.mpx.get_info_receptacle(task.pdu, task.branch, task.receptacle).await?;
    let path = format!("/pdu-{}/branch-{}/receptacle-{}", task.pdu, task.branch, task.receptacle);
    let new = info.to_mqtt(&path);
    let result = task.cache.get_modified(&new);
    task.cache = Cache::MQTTMsgList(new);

    Ok(result)
}

async fn read_branch(task: &mut Task) -> Result<MQTTMsgList, liebert::MPXError> {
    let info = task.mpx.get_info_branch(task.pdu, task.branch).await?;
    let path = format!("/pdu-{}/branch-{}", task.pdu, task.branch);
    let new = info.to_mqtt(&path);
    let result = task.cache.get_modified(&new);
    task.cache = Cache::MQTTMsgList(new);

    Ok(result)
}

async fn read_pdu(task: &mut Task) -> Result<MQTTMsgList, liebert::MPXError> {
    let info = task.mpx.get_info_pdu(task.pdu).await?;
    let path = format!("/pdu-{}", task.pdu);
    let new = info.to_mqtt(&path);
    let result = task.cache.get_modified(&new);
    task.cache = Cache::MQTTMsgList(new);

    Ok(result)
}

fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}

async fn systemd_watchdog_update(_task: &mut Task) -> Result<MQTTMsgList, liebert::MPXError> {
    let result: MQTTMsgList = Vec::new();
    let _ = sd_notify::notify(false, &[sd_notify::NotifyState::Watchdog]);
    Ok(result)
}

async fn read_events(task: &mut Task) -> Result<MQTTMsgList, liebert::MPXError> {
    let result: MQTTMsgList = Vec::new();
    let events = task.mpx.get_events().await?;

    /* 1. check if anything changed from previous state */
    match &task.cache {
        Cache::EventList(cache) => {
            if do_vecs_match(cache, &events) {
                /* no new events */
                return Ok(result)
            }
        },
        _ => { /* empty cache */ }
    }

    /* 2. handle events */
    for event in &events {
        if event.level == liebert::EventLevel::ALARM && event.event == liebert::EventType::ReceptacleOverCurrent {
            /* disable receptacle */
            eprintln!("over-current event: Disabling receptacle {}.{}.{}!", event.pdu, event.branch, event.receptacle);
            retry_cmd(&task.mpx, event.pdu, event.branch, event.receptacle, liebert::ReceptacleCmd::Disable).await;
        } else {
            println!("unhandled event: {:?}", event);
        }
    }

    /* 3. update cache */
    task.cache = Cache::EventList(events);

    Ok(result)
}

async fn setup_tasklist(mpx: std::sync::Arc<liebert::MPX>, receptacles: &liebert::ReceptacleList) -> Result<TaskList, liebert::MPXError> {
    let mut tasklist = Vec::new();

    tasklist.push(Task {
        timestamp: Instant::now(),
        timeout: Duration::from_secs(3),
        priority: TaskPriority::HIGH,
        function: |t| Box::pin(read_events(t)),
        mpx: mpx.clone(),
        pdu: 0,
        branch: 0,
        receptacle: 0,
        cache: Cache::None(()),
    });

    for r in receptacles {
        if !tasklist.contains(r.pdu, 0, 0) {
            tasklist.push(Task {
                timestamp: Instant::now(),
                timeout: Duration::from_secs(30),
                priority: TaskPriority::LOW,
                function: |t| Box::pin(read_pdu(t)),
                mpx: mpx.clone(),
                pdu: r.pdu,
                branch: 0,
                receptacle: 0,
                cache: Cache::None(()),
            });
        }

        if !tasklist.contains(r.pdu, r.branch, 0) {
            tasklist.push(Task {
                timestamp: Instant::now().checked_sub(Duration::from_secs((r.branch*10).into())).unwrap(),
                timeout: Duration::from_secs(30),
                priority: TaskPriority::LOW,
                function: |t| Box::pin(read_branch(t)),
                mpx: mpx.clone(),
                pdu: r.pdu,
                branch: r.branch,
                receptacle: 0,
                cache: Cache::None(()),
            });
        }

        tasklist.push(Task {
            timestamp: Instant::now().checked_sub(Duration::from_secs((r.branch*10).into())).unwrap(),
            timeout: Duration::from_secs(30),
            priority: TaskPriority::LOW,
            function: |t| Box::pin(read_receptacle(t)),
            mpx: mpx.clone(),
            pdu: r.pdu,
            branch: r.branch,
            receptacle: r.receptacle,
            cache: Cache::None(()),
        });
    }

    println!("Found PDU with {} receptacles...", receptacles.len());
    println!("Event polling will start in a few seconds...");

    Ok(tasklist)
}

#[derive(Debug)]
enum Command {
    Enable,
    Disable,
    Identify,
    SetLabel,
}

#[derive(Debug)]
struct Query {
    cmd: Option<Command>,
    pdu: u8,
    branch: u8,
    receptacle: u8,
    payload: Option<String>,
}

fn parse_incoming_msg(msg: rumqttc::v4::Publish) -> Query {
    let re = regex::Regex::new(r".*?/pdu-(?P<pdu>\d+)/branch-(?P<branch>\d+)/receptacle-(?P<receptacle>\d+)/control").unwrap();
    let caps = re.captures(&msg.topic).expect("received topic has not been subscribed");
    let pdu = caps["pdu"].parse::<u8>().expect("Failed to parse pdu");
    let branch = caps["branch"].parse::<u8>().expect("Failed to parse branch");
    let receptacle = caps["receptacle"].parse::<u8>().expect("Failed to parse receptacle");
    let mut cmd = match &msg.payload[..] {
        b"enable" => Some(Command::Enable),
        b"disable" => Some(Command::Disable),
        b"identify" => Some(Command::Identify),
        _ => None,
    };
    let mut payload = None;

    if cmd.is_none() {
        let re = regex::Regex::new(r"set-label (?P<label>[A-Za-z0-9-_.]+)").unwrap();
        let msgpayload = std::str::from_utf8(&msg.payload);
        if msgpayload.is_ok() {
            let caps = re.captures(msgpayload.unwrap());
            if caps.is_some() {
                payload = Some(caps.unwrap()["label"].to_string());
                cmd = Some(Command::SetLabel);
            }
        }
    }

    Query { cmd, pdu, branch, receptacle, payload }
}

struct Cfg {
    mqtt_address: String,
    mqtt_port: u16,
    mqtt_username: String,
    mqtt_password: String,
    mqtt_clientname: String,
    mqtt_prefix: String,
    mqtt_no_retained: bool,
    pdu_address: String,
    pdu_username: String,
    pdu_password: String,
}

fn get_config(filename: &str) -> Cfg {
    let cfg = match Ini::load_from_file(filename) {
        Err(ini::Error::Io(e)) => {
            eprintln!("Failed to load config file \"{}\": {}", filename, e);
            std::process::exit(1);
        },
        Err(ini::Error::Parse(e)) => {
            eprintln!("Failed to parse config file \"{}\": {}", filename, e.msg);
            std::process::exit(1);
        },
        Ok(cfg) => cfg,
    };

    let mqtt = cfg.section(Some("MQTT")).expect("MQTT section mising in config");
    let pdu = cfg.section(Some("PDU")).expect("PDU section mising in config");

    Cfg {
        mqtt_address: mqtt.get("address").expect("MQTT address missing in config").to_string(),
        mqtt_port: mqtt.get("port").expect("MQTT port missing in config").parse::<u16>().expect("Failed to parse MQTT port in config"),
        mqtt_username: mqtt.get("username").expect("MQTT username missing in config").to_string(),
        mqtt_password: mqtt.get("password").expect("MQTT password missing in config").to_string(),
        mqtt_clientname: mqtt.get("clientname").expect("MQTT client name missing in config").to_string(),
        mqtt_prefix: mqtt.get("prefix").expect("MQTT prefix missing in config").to_string(),
        mqtt_no_retained: std::str::FromStr::from_str(mqtt.get("avoid-retained").unwrap_or("false")).expect("Failed to parse avoid-retained"),

        pdu_address: pdu.get("address").expect("PDU address missing in config").to_string(),
        pdu_username: pdu.get("username").expect("PDU username missing in config").to_string(),
        pdu_password: pdu.get("password").expect("PDU password missing in config").to_string(),
    }
}

async fn retry_cmd(mpx: &liebert::MPX, pdu: u8, branch: u8, receptacle: u8, cmd: liebert::ReceptacleCmd) {
    let mut test = mpx.receptacle_command(pdu, branch, receptacle, cmd).await;

    for _i in 0..3 {
        if test.is_ok() {
            break;
        }

        test = mpx.receptacle_command(pdu, branch, receptacle, cmd).await;
    }

    if ! test.is_ok() {
        eprintln!("Failed to {:?} receptacle {} on branch {} on pdu {}", cmd, receptacle, branch, pdu);
    }
}

async fn update_label(mpx: &liebert::MPX, pdu: u8, branch: u8, receptacle: u8, label: String) {
    let info = mpx.get_info_receptacle(pdu, branch, receptacle).await;
    if info.is_err() {
        eprintln!("Failed fetch info for receptacle {}.{}.{}", pdu, branch, receptacle);
        return;
    }
    let info = info.unwrap();

    let settings = liebert::ReceptacleSettings {
        label: label,
        ..info.settings
    };

    let mut test = mpx.set_receptacle_settings(pdu, branch, receptacle, &settings).await;

    for _i in 0..3 {
        if test.is_ok() {
            break;
        }

        test = mpx.set_receptacle_settings(pdu, branch, receptacle, &settings).await;
    }

    if ! test.is_ok() {
        eprintln!("Failed to update label for receptacle {}.{}.{}", pdu, branch, receptacle);
    }
}

fn is_ready(tasklist: &mut TaskList) -> bool {
    for task in tasklist {
        /* ignore tasks not involving PDU requests */
        if task.pdu == 0 {
            continue;
        }

        if task.cache.is_none() {
            return false;
        }
    }

    true
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("{} <config-file>", args[0]);
        std::process::exit(1);
    }

    /* Config */
    let cfg = get_config(&args[1]);
    let prefix = cfg.mqtt_prefix.clone();
    let no_retained = cfg.mqtt_no_retained;

    /* MQTT */
    let mut mqttoptions = MqttOptions::new(cfg.mqtt_clientname, cfg.mqtt_address, cfg.mqtt_port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    mqttoptions.set_credentials(cfg.mqtt_username, cfg.mqtt_password);
    mqttoptions.set_transport(rumqttc::Transport::Tls(rumqttc::TlsConfiguration::default()));
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    /* PDU */
    let mpx = liebert::MPX::new(&cfg.pdu_address, &cfg.pdu_username, &cfg.pdu_password);
    let refmpx = std::sync::Arc::new(mpx);
    let receptacles = refmpx.clone().get_receptacles().await.expect("Failed to get receptacle list");
    if receptacles.len() < 1 {
        eprintln!("Found PDU without any receptacles, maybe it's still initializing?");
        std::process::exit(1);
    }
    let mut tasklist = setup_tasklist(refmpx.clone(), &receptacles).await.unwrap();
    tasklist.append_systemd_watchdog(refmpx.clone());

    /* Register control topics */
    let topic = format!("{}/+/+/+/control", cfg.mqtt_prefix);
    client.subscribe(topic, QoS::AtMostOnce).await.expect("failed to subscribe control topic");

    let (tx, mut rx) = mpsc::channel(256);

    let mut ready = false;

    tokio::spawn(async move {
        loop {
            let notification = eventloop.poll().await.unwrap();
            match notification {
                rumqttc::Event::Incoming(pkg) => {
                    match pkg {
                        rumqttc::v4::Packet::Publish(publishpkg) => {
                            let query = parse_incoming_msg(publishpkg);
                            tx.send(query).await.expect("failed to forward MQTT command");
                        },
                        _ => {}
                    }
                },
                rumqttc::Event::Outgoing(_o) => {},
            }
        }
    });

    tokio::spawn(async move {
        loop {
            /* 1. check if we can send the ready signal to systemd */
            if !ready {
                ready = is_ready(&mut tasklist);
                if ready {
                    println!("Polled all PDU data once; notifying to ready state...");
                    let _ = sd_notify::notify(false, &[sd_notify::NotifyState::Ready]);
                }
            }

            /* 2. check if any high priority task needs to be run */
            for task in &mut tasklist {
                if task.priority != TaskPriority::HIGH {
                    continue;
                }

                if task.timed_out() {
                    task.run().await.expect("failed to run task");
                }
            }

            /* 3. handle control commands received via MQTT */
            let recvq = rx.try_recv();
            if recvq.is_ok() {
                let query = recvq.unwrap();
                match query.cmd {
                    Some(Command::Enable) => {
                        println!("Enable Receptacle {}.{}.{}", query.pdu, query.branch, query.receptacle);
                        retry_cmd(&refmpx, query.pdu, query.branch, query.receptacle, liebert::ReceptacleCmd::Enable).await;
                        tasklist.reschedule_in(query.pdu, query.branch, query.receptacle, 5);
                    },
                    Some(Command::Disable) => {
                        println!("Disable Receptacle {}.{}.{}", query.pdu, query.branch, query.receptacle);
                        retry_cmd(&refmpx, query.pdu, query.branch, query.receptacle, liebert::ReceptacleCmd::Disable).await;
                        tasklist.reschedule_in(query.pdu, query.branch, query.receptacle, 5);
                    },
                    Some(Command::Identify) => {
                        retry_cmd(&refmpx, query.pdu, query.branch, query.receptacle, liebert::ReceptacleCmd::Identify).await;
                    },
                    Some(Command::SetLabel) => {
                        let label = query.payload.unwrap_or("".to_string());
                        println!("Set Receptacle {}.{}.{} label to \"{}\"", query.pdu, query.branch, query.receptacle, label);
                        update_label(&refmpx, query.pdu, query.branch, query.receptacle, label).await;
                        tasklist.reschedule_in(query.pdu, query.branch, query.receptacle, 5);
                    },
                    None => { /* ignore */},
                }
            }

            /* 4. check if oldest task needs to be executed, otherwise sleep for a second */
            let oldest = tasklist.get_oldest().unwrap();
            if oldest.timed_out() {
                let messages = oldest.run().await.expect("failed to run task");
                for msg in messages {
                    if no_retained {
                        client.publish(format!("{}{}", prefix, msg.topic), QoS::AtLeastOnce, false, msg.payload).await.unwrap();
                    } else {
                        client.publish(format!("{}{}", prefix, msg.topic), QoS::AtLeastOnce, msg.retained, msg.payload).await.unwrap();
                    }
                }
            } else {
                sleep(Duration::from_secs(1));
            }
        }
    });

    loop{
        sleep(Duration::from_secs(60));
    }
}
