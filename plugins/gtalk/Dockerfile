FROM appleboy/drone-gtalk as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="appleboy/drone-gtalk"
ENV DRONELAB_UPSTREAM_AUTHOR="appleboy"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/appleboy/drone-gtalk"
ENV DRONELAB="/drone-gtalk"
