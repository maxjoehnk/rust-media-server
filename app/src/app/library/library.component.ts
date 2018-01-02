import { Component } from '@angular/core';
import { Observable } from 'rxjs/Observable';
import { Album } from './album.model';
import { LibraryService } from './library.service';

@Component({
    selector: 'rms-library',
    templateUrl: './library.component.html',
    styleUrls: ['./library.component.scss']
})
export class LibraryComponent {
    albums$: Observable<Album[]>;

    constructor(private library: LibraryService) {
        this.albums$ = this.library.getAlbums();
    }
}
