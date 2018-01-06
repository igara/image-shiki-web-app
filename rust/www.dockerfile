FROM rust:1.22.1

ADD ./www /rust/www
WORKDIR /rust/www/

RUN apt-get update && apt-get install -y libpython3.5-dev

RUN chmod 777 /rust/www/www.sh
CMD ["/rust/www/www.sh"]

VOLUME /rust/www/
VOLUME /usr/local/cargo/
