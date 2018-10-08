FROM target/captains-log as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="target/captains-log"
ENV DRONELAB_UPSTREAM_AUTHOR="Target"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/target/captains-log"
ENV DRONELAB="node /opt/index.js"
