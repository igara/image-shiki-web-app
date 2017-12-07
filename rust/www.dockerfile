FROM rust:1.22.1

WORKDIR /var/www/rust/www

CMD ["cargo", "run"]
