import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs/Observable';
import { Track } from '../library/album.model';

export interface PlayerState {
    playing: boolean,
    current?: Track
}

@Injectable()
export class PlayerService {

    constructor(private http: HttpClient) {
    }

    play(): Observable<void> {
        return this.http.post<void>('/api/player/play', null);
    }

    pause(): Observable<void> {
        return this.http.post<void>('/api/player/pause', null);
    }

    next(): Observable<void> {
        return this.http.post<void>('/api/player/next', null);
    }

    prev(): Observable<void> {
        return this.http.post<void>('/api/player/prev', null);
    }

    getState(): Observable<PlayerState> {
        return this.http.get<PlayerState>('/api/player');
    }
}
