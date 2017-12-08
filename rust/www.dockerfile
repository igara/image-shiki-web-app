FROM rust:1.22.1

WORKDIR /var/www/
COPY www/Cargo.toml /var/www
COPY www/www.sh /var/www
COPY www/src /var/www

VOLUME /var/www/
VOLUME /usr/local/cargo/

CMD ["sh", "www.sh"]
