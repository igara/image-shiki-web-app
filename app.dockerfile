FROM busybox

ADD ./ /var/www

RUN ["true"]

VOLUME /var/www
