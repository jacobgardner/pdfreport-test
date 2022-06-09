const mainToken = 'PDFGeneration';

module.exports = {
  ENV_DEBUG: {source: 'ENV_DEBUG', default: false, runtime: false},
  NODE_ENV: {source: 'NODE_ENV', default: 'production', choices: ['production', 'test', 'development'], runtime: false},

  BASE_PATH: {source: `API.${mainToken}.BASE_PATH`},

  AWS_REGION: {source: 'AWS.REGION', runtime: false},
  AWS_IAM_DEVOPS_KEY: {source: 'AWS.IAM.DevOps.KEY', runtime: false},
  AWS_IAM_DEVOPS_SECRET: {source: 'AWS.IAM.DevOps.SECRET', runtime: false},
  AWS_ECS_CLUSTER_NAME: {source: 'AWS.ECS.CLUSTER_NAME', runtime: false},
  AWS_ECS_SERVICE_NAME: {source: `API.${mainToken}.SERVICE_NAME`, runtime: false},
  DOCKER_IMAGE: {source: `API.${mainToken}.DOCKER.IMAGE`, runtime: false},
  DOCKER_MEMORY_RESERVATION: {source: `API.${mainToken}.DOCKER.MEMORY_RESERVATION`, parse: 'toNumber', runtime: false},
  DOCKER_CPU_SHARES: {source: `API.${mainToken}.DOCKER.CPU_SHARES`, parse: 'toNumber', runtime: false},
};
