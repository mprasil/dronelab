FROM drillster/drone-rsync as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="drillster/drone-rsync"
ENV DRONELAB_UPSTREAM_AUTHOR="drillster"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/drillster/drone-rsync"
ENV DRONELAB="/usr/local/upload.sh"
