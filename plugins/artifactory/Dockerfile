FROM athieriot/drone-artifactory as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="athieriot/drone-artifactory"
ENV DRONELAB_UPSTREAM_AUTHOR="athieriot"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/athieriot/drone-artifactory"
ENV DRONELAB="node /node/index.js"
