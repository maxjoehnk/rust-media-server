import { Component, EventEmitter, Output } from '@angular/core';

@Component({
    selector: 'rms-header',
    templateUrl: './header.component.html',
    styleUrls: [
        './header.component.scss'
    ]
})
export class HeaderComponent {

    @Output('toggle-sidenav')
    toggleSidenav = new EventEmitter<void>();

    onToggleSidenav() {
        this.toggleSidenav.emit();
    }
}
