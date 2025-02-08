pipeline {
    agent any
    environment {
        DOCKER_REGISTRY = "64.23.161.84:8082"
        DOCKER_IMAGE = "actix_web_api"
        DOCKER_TAG = "latest"
        SERVER_USER = "root"
        SERVER_IP = "64.23.161.84"
        SSH_CREDENTIALS = "ssh-server-credentials"
    }
    stages {
        stage('Checkout') {
            steps {
                echo "📥 Clonando código fuente desde GitHub..."
                git branch: 'develop', credentialsId: 'github-credentials', url: 'https://github.com/rminayaro/ActixWeb.git'
            }
        }
        stage('Build Docker Image') {
            steps {
                echo "🔨 Construyendo imagen Docker..."
                bat "docker build -t $DOCKER_REGISTRY/$DOCKER_IMAGE:$DOCKER_TAG ."
            }
        }
        stage('Login to Nexus') {
            steps {
                echo "🔑 Iniciando sesión en Nexus..."
                bat "docker login -u admin -p '123456' $DOCKER_REGISTRY"
            }
        }
        stage('Push to Nexus') {
            steps {
                echo "📤 Subiendo imagen a Nexus..."
                bat "docker push $DOCKER_REGISTRY/$DOCKER_IMAGE:$DOCKER_TAG"
            }
        }
        stage('Deploy to Server') {
            steps {
                echo "🚀 Desplegando aplicación en el servidor..."
                script {
                    sshagent(credentials: [SSH_CREDENTIALS]) {
                        sh """
                        ssh -o StrictHostKeyChecking=no $SERVER_USER@$SERVER_IP << 'ENDSSH'
                        docker pull $DOCKER_REGISTRY/$DOCKER_IMAGE:$DOCKER_TAG
                        docker stop $DOCKER_IMAGE || true
                        docker rm -f $DOCKER_IMAGE || true
                        docker run -d --restart unless-stopped --name $DOCKER_IMAGE -p 8080:8080 \
                        $DOCKER_REGISTRY/$DOCKER_IMAGE:$DOCKER_TAG
                        exit
                        ENDSSH
                        """
                    }
                }
            }
        }
    }
    post {
        success {
            echo "🎉 Despliegue exitoso de Rust API!"
        }
        failure {
            echo "🚨 ERROR en el despliegue!"
        }
    }
}