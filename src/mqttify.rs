use crate::MQTTMsgList;
use crate::MQTTMsg;

pub trait ToMQTT {
    fn to_mqtt(self, prefix: &str) -> MQTTMsgList;
}

impl ToMQTT for liebert_mpx::PDUHardware {
    fn to_mqtt(self, prefix: &str) -> MQTTMsgList {
        let mut result : MQTTMsgList = Vec::new();

        result.push(MQTTMsg {
            topic: format!("{}/pem-model", prefix),
            payload: format!("{:?}", self.pem_model),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/fw-version", prefix),
            payload: format!("{}", self.fw_version),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/serial-number", prefix),
            payload: format!("{}", self.serial_number),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/wiring-type", prefix),
            payload: format!("{}", self.wiring_type),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/rated-input-voltage", prefix),
            payload: format!("{}", self.rated_input_voltage*1000 as u32),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/rated-input-current", prefix),
            payload: format!("{}", self.rated_input_current*1000 as u32),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/rated-input-line-frequency", prefix),
            payload: format!("{}", self.rated_input_line_frequency*10 as u32),
            retained: true,
        });

        result
    }
}

impl ToMQTT for liebert_mpx::PDUSettings {
    fn to_mqtt(self, prefix: &str) -> MQTTMsgList {
        let mut result : MQTTMsgList = Vec::new();

        result.push(MQTTMsg {
            topic: format!("{}/label", prefix),
            payload: format!("{}", self.label),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/asset-tag-1", prefix),
            payload: format!("{}", self.asset_tag_1),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/asset-tag-2", prefix),
            payload: format!("{}", self.asset_tag_2),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/n-over-current-alarm-threshold", prefix),
            payload: format!("{}", self.n_over_current_alarm_threshold),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/n-over-current-warning-threshold", prefix),
            payload: format!("{}", self.n_over_current_warning_threshold),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/l1-low-current-alarm-threshold", prefix),
            payload: format!("{}", self.l1_low_current_alarm_threshold),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/l1-over-current-alarm-threshold", prefix),
            payload: format!("{}", self.l1_over_current_alarm_threshold),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/l1-over-current-warning-threshold", prefix),
            payload: format!("{}", self.l1_over_current_warning_threshold),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/l2-low-current-alarm-threshold", prefix),
            payload: format!("{}", self.l2_low_current_alarm_threshold),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/l2-over-current-alarm-threshold", prefix),
            payload: format!("{}", self.l2_over_current_alarm_threshold),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/l2-over-current-warning-threshold", prefix),
            payload: format!("{}", self.l2_over_current_warning_threshold),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/l3-low-current-alarm-threshold", prefix),
            payload: format!("{}", self.l3_low_current_alarm_threshold),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/l3-over-current-alarm-threshold", prefix),
            payload: format!("{}", self.l3_over_current_alarm_threshold),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/l3-over-current-warning-threshold", prefix),
            payload: format!("{}", self.l3_over_current_warning_threshold),
            retained: true,
        });

        result
    }
}

impl ToMQTT for liebert_mpx::PDUEvents {
    fn to_mqtt(self, prefix: &str) -> MQTTMsgList {
        let mut result : MQTTMsgList = Vec::new();

        result.push(MQTTMsg {
            topic: format!("{}/l1-low-voltage", prefix),
            payload: format!("{:?}", self.low_voltage_l1),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/l2-low-voltage", prefix),
            payload: format!("{:?}", self.low_voltage_l2),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/l3-low-voltage", prefix),
            payload: format!("{:?}", self.low_voltage_l3),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/l1-over-current", prefix),
            payload: format!("{:?}", self.over_current_l1),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/l2-over-current", prefix),
            payload: format!("{:?}", self.over_current_l2),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/l3-over-current", prefix),
            payload: format!("{:?}", self.over_current_l3),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/n-over-current", prefix),
            payload: format!("{:?}", self.over_current_n),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/l1-low-current", prefix),
            payload: format!("{:?}", self.low_current_l1),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/l2-low-current", prefix),
            payload: format!("{:?}", self.low_current_l2),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/l3-low-current", prefix),
            payload: format!("{:?}", self.low_current_l3),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/failure", prefix),
            payload: format!("{:?}", self.failure),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/communication-fail", prefix),
            payload: format!("{:?}", self.communication_fail),
            retained: true,
        });

        result
    }
}

impl ToMQTT for liebert_mpx::PDUStatus {
    fn to_mqtt(self, prefix: &str) -> MQTTMsgList {
        let mut result : MQTTMsgList = Vec::new();

        result.push(MQTTMsg {
            topic: format!("{}/accumulated-energy", prefix),
            payload: format!("{}", (self.accumulated_energy * 1000.0) as u32),
            retained: false,
        });

        result.push(MQTTMsg {
            topic: format!("{}/input-power", prefix),
            payload: format!("{}", (self.input_power * 1000.0) as u32),
            retained: false,
        });

        result.push(MQTTMsg {
            topic: format!("{}/l1-voltage", prefix),
            payload: format!("{}", (self.voltage_l1_n * 1000.0) as u32),
            retained: false,
        });

        result.push(MQTTMsg {
            topic: format!("{}/l2-voltage", prefix),
            payload: format!("{}", (self.voltage_l2_n * 1000.0) as u32),
            retained: false,
        });

        result.push(MQTTMsg {
            topic: format!("{}/l3-voltage", prefix),
            payload: format!("{}", (self.voltage_l3_n * 1000.0) as u32),
            retained: false,
        });

        result.push(MQTTMsg {
            topic: format!("{}/l1-current", prefix),
            payload: format!("{}", (self.current_l1 * 1000.0) as u32),
            retained: false,
        });

        result.push(MQTTMsg {
            topic: format!("{}/l2-current", prefix),
            payload: format!("{}", (self.current_l2 * 1000.0) as u32),
            retained: false,
        });

        result.push(MQTTMsg {
            topic: format!("{}/l2-current", prefix),
            payload: format!("{}", (self.current_l2 * 1000.0) as u32),
            retained: false,
        });

        result.push(MQTTMsg {
            topic: format!("{}/n-current", prefix),
            payload: format!("{}", (self.current_n * 1000.0) as u32),
            retained: false,
        });

        result.push(MQTTMsg {
            topic: format!("{}/l1-current-available-to-alarm", prefix),
            payload: format!("{}", (self.current_available_to_alarm_l1 * 1000.0) as u32),
            retained: false,
        });

        result.push(MQTTMsg {
            topic: format!("{}/l2-current-available-to-alarm", prefix),
            payload: format!("{}", (self.current_available_to_alarm_l2 * 1000.0) as u32),
            retained: false,
        });

        result.push(MQTTMsg {
            topic: format!("{}/l3-current-available-to-alarm", prefix),
            payload: format!("{}", (self.current_available_to_alarm_l3 * 1000.0) as u32),
            retained: false,
        });

        result.push(MQTTMsg {
            topic: format!("{}/l1-current-utilization", prefix),
            payload: format!("{}", (self.current_utilization_l1 * 10.0) as u32),
            retained: false,
        });

        result.push(MQTTMsg {
            topic: format!("{}/l2-current-utilization", prefix),
            payload: format!("{}", (self.current_utilization_l2 * 10.0) as u32),
            retained: false,
        });

        result.push(MQTTMsg {
            topic: format!("{}/l3-current-utilization", prefix),
            payload: format!("{}", (self.current_utilization_l3 * 10.0) as u32),
            retained: false,
        });

        result.push(MQTTMsg {
            topic: format!("{}/line-frequency", prefix),
            payload: format!("{}", (self.line_frequency * 10.0) as u32),
            retained: false,
        });

        result
    }
}

impl ToMQTT for liebert_mpx::PDUInfo {
    fn to_mqtt(self, prefix: &str) -> MQTTMsgList {
        let mut result : MQTTMsgList = Vec::new();

        result.append(&mut self.status.to_mqtt(&format!("{}/status", prefix)));
        result.append(&mut self.events.to_mqtt(&format!("{}/events", prefix)));
        result.append(&mut self.settings.to_mqtt(&format!("{}/settings", prefix)));
        result.append(&mut self.hardware.to_mqtt(&format!("{}/hardware", prefix)));

        result
    }
}

impl ToMQTT for liebert_mpx::BranchHardware {
    fn to_mqtt(self, prefix: &str) -> MQTTMsgList {
        let mut result : MQTTMsgList = Vec::new();

        result.push(MQTTMsg {
            topic: format!("{}/brm-model", prefix),
            payload: format!("{:?}", self.brm_model),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/fw-version", prefix),
            payload: format!("{}", self.fw_version),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/serial-number", prefix),
            payload: format!("{}", self.serial_number),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/receptacle-type", prefix),
            payload: format!("{}", self.receptacle_type),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/capabilities", prefix),
            payload: format!("{}", self.capabilities),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/line-source", prefix),
            payload: format!("{}", self.line_source),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/rated-line-voltage", prefix),
            payload: format!("{}", self.rated_line_voltage*1000),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/rated-line-current", prefix),
            payload: format!("{}", self.rated_line_current*1000),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/rated-line-frequency", prefix),
            payload: format!("{}", self.rated_line_frequency*10),
            retained: true,
        });

        result
    }
}

impl ToMQTT for liebert_mpx::BranchSettings {
    fn to_mqtt(self, prefix: &str) -> MQTTMsgList {
        let mut result : MQTTMsgList = Vec::new();

        result.push(MQTTMsg {
            topic: format!("{}/label", prefix),
            payload: format!("{}", self.label),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/asset-tag-1", prefix),
            payload: format!("{}", self.asset_tag_1),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/asset-tag-2", prefix),
            payload: format!("{}", self.asset_tag_2),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/over-current-alarm-threshold", prefix),
            payload: format!("{}", self.over_current_alarm_threshold),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/over-current-warning-threshold", prefix),
            payload: format!("{}", self.over_current_warning_threshold),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/low-current-alarm-threshold", prefix),
            payload: format!("{}", self.low_current_alarm_threshold),
            retained: true,
        });

        result
    }
}

impl ToMQTT for liebert_mpx::BranchEvents {
    fn to_mqtt(self, prefix: &str) -> MQTTMsgList {
        let mut result : MQTTMsgList = Vec::new();

        result.push(MQTTMsg {
            topic: format!("{}/low-voltage", prefix),
            payload: format!("{:?}", self.low_voltage),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/over-current", prefix),
            payload: format!("{:?}", self.over_current),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/low-current", prefix),
            payload: format!("{:?}", self.low_current),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/failure", prefix),
            payload: format!("{:?}", self.failure),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/breaker-open", prefix),
            payload: format!("{:?}", self.breaker_open),
            retained: true,
        });

        result
    }
}

impl ToMQTT for liebert_mpx::BranchStatus {
    fn to_mqtt(self, prefix: &str) -> MQTTMsgList {
        let mut result : MQTTMsgList = Vec::new();

        result.push(MQTTMsg {
            topic: format!("{}/accumulated-energy", prefix),
            payload: format!("{}", (self.accumulated_energy * 1000.0) as u32),
            retained: false,
        });

        result.push(MQTTMsg {
            topic: format!("{}/voltage", prefix),
            payload: format!("{}", (self.voltage * 1000.0) as u32),
            retained: false,
        });

        result.push(MQTTMsg {
            topic: format!("{}/current", prefix),
            payload: format!("{}", (self.current * 1000.0) as u32),
            retained: false,
        });

        result.push(MQTTMsg {
            topic: format!("{}/current-available-to-alarm", prefix),
            payload: format!("{}", (self.current_available_to_alarm * 1000.0) as u32),
            retained: false,
        });

        result.push(MQTTMsg {
            topic: format!("{}/current-utilization", prefix),
            payload: format!("{}", (self.current_utilization * 10.0) as u32),
            retained: false,
        });

        result.push(MQTTMsg {
            topic: format!("{}/power", prefix),
            payload: format!("{}", (self.power * 1000.0) as u32),
            retained: false,
        });

        result.push(MQTTMsg {
            topic: format!("{}/apparent-power", prefix),
            payload: format!("{}", (self.apparent_power * 1000.0) as u32),
            retained: false,
        });

        result.push(MQTTMsg {
            topic: format!("{}/power-factor", prefix),
            payload: format!("{}", self.apparent_power),
            retained: false,
        });

        result
    }
}

impl ToMQTT for liebert_mpx::BranchInfo {
    fn to_mqtt(self, prefix: &str) -> MQTTMsgList {
        let mut result : MQTTMsgList = Vec::new();

        result.append(&mut self.status.to_mqtt(&format!("{}/status", prefix)));
        result.append(&mut self.events.to_mqtt(&format!("{}/events", prefix)));
        result.append(&mut self.settings.to_mqtt(&format!("{}/settings", prefix)));
        result.append(&mut self.hardware.to_mqtt(&format!("{}/hardware", prefix)));

        result
    }
}

impl ToMQTT for liebert_mpx::ReceptacleHardware {
    fn to_mqtt(self, prefix: &str) -> MQTTMsgList {
        let mut result : MQTTMsgList = Vec::new();

        result.push(MQTTMsg {
            topic: format!("{}/receptacle-type", prefix),
            payload: format!("{}", self.receptacle_type),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/line-source", prefix),
            payload: format!("{}", self.line_source),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/capabilities", prefix),
            payload: format!("{}", self.capabilities),
            retained: true,
        });

        result
    }
}

impl ToMQTT for liebert_mpx::ReceptacleSettings {
    fn to_mqtt(self, prefix: &str) -> MQTTMsgList {
        let mut result : MQTTMsgList = Vec::new();

        result.push(MQTTMsg {
            topic: format!("{}/label", prefix),
            payload: format!("{}", self.label),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/asset-tag-1", prefix),
            payload: format!("{}", self.asset_tag_1),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/asset-tag-2", prefix),
            payload: format!("{}", self.asset_tag_2),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/over-current-alarm-threshold", prefix),
            payload: format!("{}", self.over_current_alarm_threshold),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/over-current-warning-threshold", prefix),
            payload: format!("{}", self.over_current_warning_threshold),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/low-current-alarm-threshold", prefix),
            payload: format!("{}", self.low_current_alarm_threshold),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/power-state", prefix),
            payload: format!("{}", if self.power_state { "on" } else { "off" }),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/power-control", prefix),
            payload: format!("{}", if self.power_control { "on" } else { "off" }),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/power-control-locked", prefix),
            payload: format!("{}", if self.control_lock_state { "on" } else { "off" }),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/power-on-delay", prefix),
            payload: format!("{}", self.power_on_delay),
            retained: true,
        });

        result
    }
}

impl ToMQTT for liebert_mpx::ReceptacleEvents {
    fn to_mqtt(self, prefix: &str) -> MQTTMsgList {
        let mut result : MQTTMsgList = Vec::new();

        result.push(MQTTMsg {
            topic: format!("{}/over-current", prefix),
            payload: format!("{:?}", self.over_current),
            retained: true,
        });

        result.push(MQTTMsg {
            topic: format!("{}/low-current", prefix),
            payload: format!("{:?}", self.low_current),
            retained: true,
        });

        result
    }
}

impl ToMQTT for liebert_mpx::ReceptacleStatus {
    fn to_mqtt(self, prefix: &str) -> MQTTMsgList {
        let mut result : MQTTMsgList = Vec::new();

        result.push(MQTTMsg {
            topic: format!("{}/accumulated-energy", prefix),
            payload: format!("{}", (self.accumulated_energy * 1000.0) as u32),
            retained: false,
        });

        result.push(MQTTMsg {
            topic: format!("{}/voltage", prefix),
            payload: format!("{}", (self.voltage * 1000.0) as u32),
            retained: false,
        });

        result.push(MQTTMsg {
            topic: format!("{}/current", prefix),
            payload: format!("{}", (self.current * 1000.0) as u32),
            retained: false,
        });

        result.push(MQTTMsg {
            topic: format!("{}/current-available-to-alarm", prefix),
            payload: format!("{}", (self.current_available_to_alarm * 1000.0) as u32),
            retained: false,
        });

        result.push(MQTTMsg {
            topic: format!("{}/current-utilization", prefix),
            payload: format!("{}", (self.current_utilization * 10.0) as u32),
            retained: false,
        });

        result.push(MQTTMsg {
            topic: format!("{}/power", prefix),
            payload: format!("{}", (self.power * 1000.0) as u32),
            retained: false,
        });

        result.push(MQTTMsg {
            topic: format!("{}/apparent-power", prefix),
            payload: format!("{}", (self.apparent_power * 1000.0) as u32),
            retained: false,
        });

        result.push(MQTTMsg {
            topic: format!("{}/power-factor", prefix),
            payload: format!("{}", self.apparent_power),
            retained: false,
        });

        result.push(MQTTMsg {
            topic: format!("{}/current-crest-factor", prefix),
            payload: format!("{}", self.current_crest_factor),
            retained: false,
        });

        result
    }
}

impl ToMQTT for liebert_mpx::ReceptacleInfo {
    fn to_mqtt(self, prefix: &str) -> MQTTMsgList {
        let mut result : MQTTMsgList = Vec::new();

        result.append(&mut self.status.to_mqtt(&format!("{}/status", prefix)));
        result.append(&mut self.events.to_mqtt(&format!("{}/events", prefix)));
        result.append(&mut self.settings.to_mqtt(&format!("{}/settings", prefix)));
        result.append(&mut self.hardware.to_mqtt(&format!("{}/hardware", prefix)));

        result
    }
}
