#!/bin/bash -x

PID_FILE="/tmp/test_dnsmasq.pid"
LEASE_FILE="/tmp/test_dnsmasq.lease"
IPV4_BLOCK="192.0.2"
DHCP_SRV_IP="${IPV4_BLOCK}.1"


if [ -e $PID_FILE ];then
    kill `cat $PID_FILE`
fi

if [ "CHK$1" == "CHKrm" ]; then
    ip link del dhcpcli
    ip netns del mozim
    exit 0
fi

ip netns add mozim
ip link add dhcpcli type veth peer name dhcpsrv
ip link set dhcpcli up
ip link set dhcpsrv netns mozim
ip netns exec mozim ip link set dhcpsrv up
ip netns exec mozim ip addr add ${DHCP_SRV_IP}/24 dev dhcpsrv
rm $LEASE_FILE -f
ip netns exec mozim dnsmasq \
    --log-dhcp \
    --conf-file=/dev/null \
    --dhcp-leasefile=$LEASE_FILE \
    --no-hosts \
    --dhcp-host=dummy-host,${IPV4_BLOCK}.99 \
    --dhcp-option=option:dns-server,8.8.8.8,1.1.1.1 \
    --dhcp-option=option:mtu,1492 \
    --dhcp-option=option:domain-name,example.com\
    --dhcp-option=option:ntp-server,${DHCP_SRV_IP} \
    --bind-interfaces \
    --except-interface=lo \
    --clear-on-reload \
    --pid-file=$PID_FILE \
    --listen-address=$DHCP_SRV_IP \
    --dhcp-range=${IPV4_BLOCK}.2,${IPV4_BLOCK}.50,60 --no-ping &
sleep 5
