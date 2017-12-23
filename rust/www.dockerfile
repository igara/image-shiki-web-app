FROM rust:1.22.1

ADD ./www /rust/www
WORKDIR /rust/www/

RUN chmod 777 /rust/www/www.sh
CMD ["/rust/www/www.sh"]

VOLUME /rust/www/
VOLUME /usr/local/cargo/
