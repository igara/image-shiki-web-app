FROM node:alpine

ADD ./ /nodejs
WORKDIR /nodejs
RUN chmod 777 /nodejs/
RUN chmod 777 /nodejs/npm_build.sh
RUN chmod 777 /nodejs/parcel_build.sh
CMD ["sh", "/nodejs/npm_build.sh"]

VOLUME /nodejs
