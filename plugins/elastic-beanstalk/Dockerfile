FROM peloton/drone-elastic-beanstalk as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="peloton/drone-elastic-beanstalk"
ENV DRONELAB_UPSTREAM_AUTHOR="josmo"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/josmo/drone-elastic-beanstalk"
ENV DRONELAB="/bin/drone-elastic-beanstalk"
