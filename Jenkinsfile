pipeline {
  agent any
  triggers {
    githubPush()
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