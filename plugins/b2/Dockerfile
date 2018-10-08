FROM techknowlogick/drone-b2 as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="techknowlogick/drone-b2"
ENV DRONELAB_UPSTREAM_AUTHOR="techknowlogick"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/techknowlogick/drone-b2"
ENV DRONELAB="/bin/drone-b2"
