FROM appleboy/drone-jenkins as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="appleboy/drone-jenkins"
ENV DRONELAB_UPSTREAM_AUTHOR="appleboy"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/appleboy/drone-jenkins"
ENV DRONELAB="/drone-jenkins"
