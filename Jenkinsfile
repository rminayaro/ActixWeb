pipeline {
    agent any

    stages {
        stage('Descargar Código') {
            steps {
                git branch: 'main', url: 'https://github.com/rminayaro/ActixWeb.git'
            }
        }

        stage('Construir y Probar') {
            steps {
                bat 'echo "Compilando código..."'
                bat 'echo "Ejecutando pruebas..."'
            }
        }

        stage('Verificar Cargo.lock') {
            steps {
                // Verificamos que Cargo.lock esté presente en el directorio del repositorio
                script {
                    // Listar archivos en el directorio actual para verificar Cargo.lock
                    echo "Listando archivos en el directorio actual..."
                    bat 'dir'
                }
            }
        }

        stage('Construir Imagen Docker') {
            steps {
                script {
                    bat 'docker build -t tuusuario/tuimagen:version .'
                    bat 'docker save -o tuimagen.tar tuusuario/tuimagen:version'  // Guardar la imagen en un archivo
                }
            }
        }


 stages {
        stage('Build Docker Image') {
            steps {
                script {
                    // Construir la imagen Docker
                    sh 'docker build -t mi_proyecto_dockerizado-actix_web_api:latest .'
                }
            }
        }

        stage('Login to Nexus Docker Repository') {
            steps {
                script {
                    // Inicia sesión en el repositorio Docker de Nexus usando Docker CLI
                    sh 'echo "Minaya022005" | docker login -u rminayaro --password-stdin http://localhost:8081/repository/rminaya/'
                }
            }
        }

        stage('Tag Docker Image') {
            steps {
                script {
                    // Etiquetar la imagen Docker para el repositorio Nexus
                    sh 'docker tag mi_proyecto_dockerizado-actix_web_api:latest localhost:8081/repository/rminaya/miimagen:1.0'
                }
            }
        }

        stage('Push Docker Image to Nexus') {
            steps {
                script {
                    // Empujar la imagen Docker a Nexus
                    sh 'docker push localhost:8081/repository/rminaya/miimagen:1.0'
                }
            }
        }
    }



        stage('Desplegar en Servidor') {
            when {
                branch 'main'
            }
            steps {
                bat 'echo "Desplegando en el servidor..."'
                // Aquí puedes agregar comandos para desplegar la imagen Dockerss en tu servidor
            }
        }
    }
}
