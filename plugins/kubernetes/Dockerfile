FROM quay.io/honestbee/drone-kubernetes as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="quay.io/honestbee/drone-kubernetes"
ENV DRONELAB_UPSTREAM_AUTHOR="mactynow"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/honestbee/drone-kubernetes"
ENV DRONELAB="/bin/bash"
