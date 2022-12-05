#!/bin/bash -x

PID_FILE="/tmp/test_dnsmasq.pid"
LEASE_FILE="/tmp/test_dnsmasq.lease"
IPV4_BLOCK="192.0.2"
DHCP_SRV_IP="${IPV4_BLOCK}.1"

if [ "CHK$1" == "CHKrm" ]; then
    if [ -e $PID_FILE ];then
        kill `cat $PID_FILE`
    fi
    ip link del eth1
    ip netns del mozim
    exit 0
fi

ip netns add mozim
ip link add eth1 type veth peer name eth1.ep
ip link set eth1 up
ip link set eth1.ep netns mozim
ip netns exec mozim ip link set eth1.ep up
ip netns exec mozim ip addr add ${DHCP_SRV_IP}/24 dev eth1.ep
rm $LEASE_FILE -f
ip netns exec mozim dnsmasq \
    --log-dhcp \
    --keep-in-foreground \
    --no-daemon \
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
    --listen-address=$DHCP_SRV_IP \
    --dhcp-range=${IPV4_BLOCK}.2,${IPV4_BLOCK}.50,60 --no-ping &
sleep 5
