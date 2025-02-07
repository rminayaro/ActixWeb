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


        stage('Subir a Nexus') {
    steps {
        script {
            // Inicia sesión en Docker con tu usuario y contraseña
            bat "docker login -u rminayaro -p Minaya022005 http://localhost:8081"

            // Etiqueta la imagen Docker con el repositorio de Nexus
            bat "docker tag mi_proyecto_dockerizado-actix_web_api:latest localhost:8081/docker-releases/miimagen:1.0"

            // Empuja la imagen Docker al repositorio de Nexus
            bat "docker push localhost:8081/docker-releases/miimagen:1.0"

            // Usando nexusArtifactUploader para subir el archivo .tar
            nexusArtifactUploader(
                nexusVersion: 'nexus3',
                protocol: 'http',
                nexusUrl: 'http://localhost:8081',
                groupId: 'com.example',
                artifactId: 'miimagen',
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
