FROM ibmclouddevops/drone-bluemix-cloudfoundry as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="ibmclouddevops/drone-bluemix-cloudfoundry"
ENV DRONELAB_UPSTREAM_AUTHOR="jcantosz"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/ibmclouddevops/drone-bluemix-cloudfoundry"
ENV DRONELAB="/bin/sh -c /bin/deploy.sh"
