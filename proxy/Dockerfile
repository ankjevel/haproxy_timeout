FROM haproxy:1.9-alpine

RUN \
  apk add --update rsyslog \
    && rm -rf /var/cache/apk/*

COPY haproxy.cfg /usr/local/etc/haproxy/haproxy.cfg

CMD /sbin/syslogd -O /dev/stdout && haproxy -f /usr/local/etc/haproxy/haproxy.cfg
