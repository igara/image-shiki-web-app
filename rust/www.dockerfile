FROM rust:1.22.1

ADD ./www /rust/www
WORKDIR /rust/www/

VOLUME /rust/www/
VOLUME /usr/local/cargo/

RUN chmod 777 /rust/www/www.sh
CMD ["sh", "www.sh"]
