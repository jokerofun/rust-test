pipeline {
  agent {
      table 'rust'
  }
  stages {
    stage('Build') {
      steps {
        sh 'cargo build'
      }
    }

    stage('Test') {
      steps {
        sh 'cargo test'
      }
    }

  }
}