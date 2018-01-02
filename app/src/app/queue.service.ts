import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs/Observable';
import { Track } from './library/album.model';

@Injectable()
export class QueueService {

    constructor(private http: HttpClient) {
    }

    queue(track: Track): Observable<void> {
        return this.http.post<void>(`/api/queue/${track.id}`, null);
    }

}
