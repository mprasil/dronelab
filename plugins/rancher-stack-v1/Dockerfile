FROM dubc/drone-rancher-stack-v1 as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="dubc/drone-rancher-stack-v1"
ENV DRONELAB_UPSTREAM_AUTHOR="wayneconnolly"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/wayneconnolly/drone-rancher-stack-v1"
ENV DRONELAB="/bin/sh -c /bin/script.sh"
