import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';

import { AppRoutingModule } from './app-routing.module';

import { AppComponent } from './app.component';
import { HttpClientModule } from '@angular/common/http';
import { Apollo, ApolloModule } from 'apollo-angular';
import { HttpLink, HttpLinkModule } from 'apollo-angular-link-http';
import { InMemoryCache } from 'apollo-cache-inmemory';
import { SidenavComponent } from './sidenav/sidenav.component';
import { HeaderComponent } from './header/header.component';
import { SharedModule } from './shared/shared.module';
import { PlayerModule } from './player/player.module';
import { LibraryModule } from './library/library.module';
import { QueueService } from './queue.service';
import { PlaylistsModule } from './playlists/playlists.module';

@NgModule({
    declarations: [
        AppComponent,
        SidenavComponent,
        HeaderComponent
    ],
    imports: [
        BrowserModule,
        AppRoutingModule,
        HttpClientModule,
        HttpLinkModule,
        ApolloModule,
        SharedModule,
        PlayerModule,
        LibraryModule,
        PlaylistsModule
    ],
    providers: [
        QueueService
    ],
    bootstrap: [AppComponent]
})
export class AppModule {
    constructor(apollo: Apollo,
                httpLink: HttpLink) {
        apollo.create({
            link: httpLink.create({}),
            cache: new InMemoryCache(),
            connectToDevTools: true
        });
    }
}
