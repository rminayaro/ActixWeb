pipeline {
    agent any

    stages {
        stage('Descargar Código') {
            steps {
                git branch: 'main', url: 'pipeline {
    agent any

    stages {
        stage('Descargar Código') {
            steps {
                git branch: 'main', url: 'pipeline {
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

        stage('Construir Imagen Docker') {
            steps {
                script {
                    bat 'docker build -t tuusuario/tuimagen:version .'
                    bat 'docker save -o tuimagen.tar tuusuario/tuimagen:version'  // Guardar la imagen en un archivo
                }
            }
        }

        stage('Subir a Nexus') {
            steps {
                script {
                    nexusArtifactUploader(
                        nexusVersion: 'nexus3',
                        protocol: 'http',
                        nexusUrl: 'http://localhost:8081',
                        groupId: 'com.example',
                        artifactId: 'tuimagen',
                        version: '1.0',
                        repository: 'docker-releases',
                        credentialsId: 'nexus-credenciales',
                        extension: 'tar',
                        file: "tuimagen.tar"
                    )
                }
            }
        }

        stage('Desplegar en Servidor') {
            when {
                branch 'main'
            }
            steps {
                bat 'echo "Desplegando en el servidor..."'
                // Aquí puedes agregar comandos para desplegar la imagen Docker en tu servidor
            }
        }
    }
}
'
            }
        }

        stage('Construir y Probar') {
            steps {
                bat 'echo "Compilando código..."'
                bat 'echo "Ejecutando pruebas..."'
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

        stage('Subir a Nexus') {
            steps {
                script {
                    nexusArtifactUploader(
                        nexusVersion: 'nexus3',
                        protocol: 'http',
                        nexusUrl: 'http://localhost:8081',
                        groupId: 'com.example',
                        artifactId: 'tuimagen',
                        version: '1.0',
                        repository: 'docker-releases',
                        credentialsId: 'nexus-credenciales',
                        extension: 'tar',
                        file: "tuimagen.tar"
                    )
                }
            }
        }

        stage('Desplegar en Servidor') {
            when {
                branch 'main'
            }
            steps {
                bat 'echo "Desplegando en el servidor..."'
                // Aquí puedes agregar comandos para desplegar la imagen Docker en tu servidor
            }
        }
    }
}
'
            }
        }

        stage('Construir y Probar') {
            steps {
                bat 'echo "Compilando código..."'
                bat 'echo "Ejecutando pruebas..."'
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

        stage('Subir a Nexus') {
            steps {
                script {
                    nexusArtifactUploader(
                        nexusVersion: 'nexus3',
                        protocol: 'http',
                        nexusUrl: 'http://localhost:8081',
                        groupId: 'com.example',
                        artifactId: 'tuimagen',
                        version: '1.0',
                        repository: 'docker-releases',
                        credentialsId: 'nexus-credenciales',
                        extension: 'tar',
                        file: "tuimagen.tar"
                    )
                }
            }
        }

        stage('Desplegar en Servidor') {
            when {
                branch 'main'
            }
            steps {
                bat 'echo "Desplegando en el servidor..."'
                // Aquí puedes agregar comandos para desplegar la imagen Docker en tu servidor
            }
        }
    }
}
