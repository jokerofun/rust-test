pipeline {
  agent {
      table 'rust'
  }
  stages {
    stage('Build') {
      steps {
        bat 'cargo build'
      }
    }

    stage('Test') {
      steps {
        bat 'cargo test'
      }
    }

  }
}