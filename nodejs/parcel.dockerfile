FROM node:alpine

WORKDIR /nodejs

CMD ["/nodejs/parcel_build.sh"]
