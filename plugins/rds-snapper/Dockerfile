FROM quay.io/honestbee/rds-snapper as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="quay.io/honestbee/rds-snapper"
ENV DRONELAB_UPSTREAM_AUTHOR="tuannvm"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/https://github.com/honestbee/devops-tools/tree/master/rds-snapper"
ENV DRONELAB="/bin/rds-snapper"
