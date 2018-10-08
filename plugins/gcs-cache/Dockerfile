FROM homerovalle/drone-gcs-cache as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="homerovalle/drone-gcs-cache"
ENV DRONELAB_UPSTREAM_AUTHOR="hvalle"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/hvalle/drone-gcs-cache"
ENV DRONELAB="/bin/drone-gcs-cache"
