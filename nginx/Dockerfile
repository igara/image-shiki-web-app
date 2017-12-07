FROM nginx:alpine

ARG AGENT_UID=1000
RUN adduser -D -H -s /bin/sh -u ${AGENT_UID} www-data

ADD nginx.conf /etc/nginx/
ADD sites/ /etc/nginx/sites-available

CMD ["nginx"]

EXPOSE 80 443
