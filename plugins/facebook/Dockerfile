FROM appleboy/drone-facebook as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="appleboy/drone-facebook"
ENV DRONELAB_UPSTREAM_AUTHOR="appleboy"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/appleboy/drone-facebook"
ENV DRONELAB="/drone-facebook"
