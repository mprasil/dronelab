FROM plugins/s3 as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="plugins/s3"
ENV DRONELAB_UPSTREAM_AUTHOR="drone-plugins"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/drone-plugins/drone-s3"
ENV DRONELAB="/bin/drone-s3"
