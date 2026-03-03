package io.github.gasparkral.dnd.di

import androidx.room.Room
import io.github.gasparkral.dnd.infra.dbstruct.AppDatabase
import io.github.gasparkral.dnd.infra.service.CharacterService
import org.koin.android.ext.koin.androidContext
import org.koin.dsl.module

val appModule = module {

    // Base de datos Room — singleton
    single {
        Room.databaseBuilder(
            androidContext(),
            AppDatabase::class.java,
            "dnd-database"
        ).build()
    }

    // DAO — extraído del singleton de la BD
    single { get<AppDatabase>().characterDao() }

    // Service — recibe el DAO por constructor
    single { CharacterService(get()) }
}