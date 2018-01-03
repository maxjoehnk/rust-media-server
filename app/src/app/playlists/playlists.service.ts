import { Injectable } from '@angular/core';
import { Playlist } from './playlist.model';
import { Observable } from 'rxjs/Observable';
import { HttpClient } from '@angular/common/http';

@Injectable()
export class PlaylistsService {

    constructor(private http: HttpClient) {
    }

    getPlaylists(): Observable<Playlist[]> {
        return this.http.get<Playlist[]>('/api/library/playlists');
    }
}
