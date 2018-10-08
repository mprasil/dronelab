FROM dhoeric/drone-airbrake-deployment:0.0.1 as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="dhoeric/drone-airbrake-deployment:0.0.1"
ENV DRONELAB_UPSTREAM_AUTHOR="dhoeric"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/dhoeric/drone-airbrake-deployment"
ENV DRONELAB="/bin/drone-airbrake-deployment"
