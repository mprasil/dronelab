FROM appleboy/drone-scp as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="appleboy/drone-scp"
ENV DRONELAB_UPSTREAM_AUTHOR="appleboy"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/appleboy/drone-scp"
ENV DRONELAB="/bin/drone-scp"
