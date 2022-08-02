var env = require('wnd-env');
var gulp = require('gulp');
var apiTasks = require('wnd-gulp-api-tasks');

gulp.task('publish', function () {
  return apiTasks.aws.deployToClusterService(
    env.get('AWS_ECS_CLUSTER_NAME'),
    env.get('AWS_ECS_SERVICE_NAME'),
    env.get('DOCKER_IMAGE'),
    env.getRuntimeVars(),
    {
      region: env.get('AWS_REGION'),
      accessKeyId: env.get('AWS_IAM_DEVOPS_KEY'),
      secretAccessKey: env.get('AWS_IAM_DEVOPS_SECRET'),
    },
    {
      waitForDeployment: false,
      memoryReservation: env.get('DOCKER_MEMORY_RESERVATION'),
      cpuShares: env.get('DOCKER_CPU_SHARES'),
    }
  );
});
