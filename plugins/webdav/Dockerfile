FROM vividboarder/drone-webdav as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="vividboarder/drone-webdav"
ENV DRONELAB_UPSTREAM_AUTHOR="vividboarder"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/vividboarder/drone-webdav"
ENV DRONELAB="/bin/sh -c /bin/push.sh"
