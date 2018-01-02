import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs/Observable';
import { Album } from './album.model';

@Injectable()
export class LibraryService {

    constructor(private http: HttpClient) {
    }

    getAlbums(): Observable<Album[]> {
        return this.http.get<Album[]>('/api/library/albums');
    }

    getAlbum(id: number): Observable<Album> {
        return this.http.get<Album>(`/api/library/albums/${id}`);
    }

}
