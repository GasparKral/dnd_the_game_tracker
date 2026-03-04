import org.jetbrains.kotlin.gradle.dsl.JvmTarget

plugins {
    alias(libs.plugins.android.application)
    alias(libs.plugins.kotlin.android)
    alias(libs.plugins.kotlin.compose)
    alias(libs.plugins.kotlin.serialization)
    id("com.google.devtools.ksp")
}

android {
    namespace = "com.dndmanager"
    compileSdk = 36

    // 🔽 AGREGA ESTAS LÍNEAS PARA UNIFICAR LAS VERSIONES DE JAVA
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_21
        targetCompatibility = JavaVersion.VERSION_21
    }

    kotlinOptions {
        JvmTarget.JVM_17
    }

    defaultConfig {
        applicationId = "com.dndmanager"
        minSdk = 26
        targetSdk = 35
        versionCode = 1
        versionName = "1.0"
    }

    buildTypes {
        debug {
            // En debug apunta a localhost (cuando el móvil está en la misma red)
            buildConfigField("String", "BASE_URL", "\"http://10.0.2.2:3000\"")
        }
        release {
            isMinifyEnabled = true
            // En release apunta al túnel de Cloudflare (URL fija)
            buildConfigField("String", "BASE_URL", "\"https://tu-tunnel.trycloudflare.com\"")
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }

    buildFeatures {
        compose = true
        buildConfig = true
        viewBinding = true
    }
}

dependencies {
    implementation(libs.material)
    implementation(libs.androidx.appcompat)
    implementation(libs.androidx.constraintlayout)
    implementation(libs.androidx.navigation.fragment.ktx)
    implementation(libs.androidx.navigation.ui.ktx)
    // ── Compose BOM ──────────────────────────────────────────────────────────
    val composeBom = platform(libs.androidx.compose.bom)
    implementation(composeBom)
    implementation(libs.androidx.compose.ui)
    implementation(libs.androidx.compose.ui.tooling.preview)
    implementation(libs.androidx.compose.material3)
    implementation(libs.androidx.compose.material.icons.extended)
    debugImplementation(libs.androidx.compose.ui.tooling)

    // ── AndroidX Core ────────────────────────────────────────────────────────
    implementation(libs.androidx.core.ktx)
    implementation(libs.androidx.lifecycle.runtime.ktx)
    implementation(libs.androidx.activity.compose)

    // ── Navegación ───────────────────────────────────────────────────────────
    implementation(libs.androidx.navigation.compose)

    // ── ViewModel ────────────────────────────────────────────────────────────
    implementation(libs.androidx.lifecycle.viewmodel.compose)

    // -─ Red: Ktor Client (alternativa moderna a Retrofit, multiplatform-ready)
    // Ktor es más idiomático con corrutinas y kotlinx.serialization
    implementation(libs.ktor.client.android)
    implementation(libs.ktor.client.content.negotiation)
    implementation(libs.ktor.client.serialization)
    implementation(libs.ktor.client.logging)
    // WebSocket nativo con Ktor
    implementation(libs.ktor.client.websockets)
    implementation(libs.ktor.client.cio)

    // ── Serialización ────────────────────────────────────────────────────────
    implementation(libs.kotlinx.serialization.json)

    // ── Corrutinas ────────────────────────────────────────────────────────────
    implementation(libs.kotlinx.coroutines.android)

    // ── Inyección de dependencias ─────────────────────────────────────────────
    implementation(libs.koin.android)
    implementation(libs.koin.androidx.compose)

    // ── Persistencia local ────────────────────────────────────────────────────
    // Para guardar el token JWT y la URL del servidor
    implementation(libs.androidx.datastore.preferences)
    ksp(libs.androidx.room.compiler)
    implementation(libs.androidx.room.runtime)
    implementation(libs.androidx.room.ktx)

    // ── Markdown renderer ─────────────────────────────────────────────────────
    // Para mostrar el lore de Obsidian (notas .md) en la app
    implementation(libs.markdown)

    // ── Imágenes ──────────────────────────────────────────────────────────────
    implementation(libs.coil.compose)

    // ── Testing ───────────────────────────────────────────────────────────────
    testImplementation(libs.junit)
    androidTestImplementation(libs.androidx.junit)
    androidTestImplementation(libs.androidx.espresso.core)
    // androidTestImplementation(composeBom)
    androidTestImplementation(libs.androidx.ui.test.junit4)
}

