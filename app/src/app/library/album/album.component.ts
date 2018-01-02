import { Component } from '@angular/core';
import { Album, Track } from '../album.model';
import { ActivatedRoute } from '@angular/router';
import { Observable } from 'rxjs/Observable';
import 'rxjs/add/operator/map';
import { QueueService } from '../../queue.service';

@Component({
    selector: 'rms-album',
    templateUrl: './album.component.html',
    styleUrls: ['./album.component.scss']
})
export class AlbumComponent {

    album$: Observable<Album>;

    constructor(private route: ActivatedRoute, private queue: QueueService) {
        this.album$ = this.route.data.map(({ album }) => album);
    }

    queueTrack(track: Track) {
        this.queue
            .queue(track)
            .subscribe(() => {
            });
    }

}
