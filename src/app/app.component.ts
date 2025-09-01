import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { invoke } from '@tauri-apps/api/core';
import { MatDialog, MatDialogModule } from '@angular/material/dialog'
import { CreateDialog } from './dialogs/create-dialog/create-dialog.component';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

@Component({
  selector: 'app-root',
  imports: [
    RouterOutlet,
    CommonModule,
    MatDialogModule,
    FormsModule
  ],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent {
  constructor(private dialog: MatDialog) {}
  title = 'hedera';
  
  // placeholder
  async onButtonClick() {
    const message = await invoke<string>('show_popup');
    alert(message); // this shows the popup
  }

  openDialogCreate(){
    this.dialog.closeAll();
    this.dialog.open(CreateDialog, {
      width: '300vw',
      disableClose: true,
    })
  }
}