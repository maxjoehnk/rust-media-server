import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { PlaylistsComponent } from './playlists/playlists.component';
import { PlaylistsService } from './playlists.service';
import { RouterModule, Routes } from '@angular/router';
import { SharedModule } from '../shared/shared.module';

const routes: Routes = [
    {
        path: 'playlists',
        component: PlaylistsComponent
    }
];

@NgModule({
    imports: [
        CommonModule,
        SharedModule,
        RouterModule.forChild(routes)
    ],
    declarations: [
        PlaylistsComponent
    ],
    providers: [
        PlaylistsService
    ]
})
export class PlaylistsModule {
}
