import { Component, OnInit } from '@angular/core';
import { PlaylistsService } from '../playlists.service';
import { Observable } from 'rxjs/Observable';
import { Playlist } from '../playlist.model';

@Component({
    selector: 'rms-playlists',
    templateUrl: './playlists.component.html',
    styleUrls: ['./playlists.component.scss']
})
export class PlaylistsComponent {

    playlists$: Observable<Playlist[]>;

    constructor(private playlists: PlaylistsService) {
        this.playlists$ = playlists.getPlaylists();
    }
}
