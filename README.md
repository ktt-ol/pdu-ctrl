This is a controller daemon for Liebert MPX PDUs making its
data available on MQTT.

Features:
 * regularly poll data from pdu modules and publish to MQTT
 * regularly poll data from branch modules and publish to MQTT
 * regularly poll data from receptacles and publish to MQTT
 * command interface for receptacles
   - disable (set receptacle state to off)
   - enable (set receptacle state to on)
   - identify (blinks receptacle's LED for some seconds)
   - set-label <string> (set receptacle's label)
 * support to enable/disable/identify receptacles via MQTT
 * automatically disable receptacles on incoming over-current alarm
 * systemd notification support
   - send READY notification once everything has been initialized
   - send WATCHDOG notifications every 30 seconds
