FROM wyattjoh/drone-gcs as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="wyattjoh/drone-gcs"
ENV DRONELAB_UPSTREAM_AUTHOR="wyattjoh"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/wyattjoh/drone-gcs"
ENV DRONELAB="/bin/drone-gcs"
