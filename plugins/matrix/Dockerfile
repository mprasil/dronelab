FROM plugins/matrix as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="plugins/matrix"
ENV DRONELAB_UPSTREAM_AUTHOR="drone-plugins"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/drone-plugins/drone-matrix"
ENV DRONELAB="/bin/drone-matrix"
