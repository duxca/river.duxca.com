import { test, expect } from '@playwright/test';

test.describe('River Management', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
  });

  test('should be able to add a new river', async ({ page }) => {
    // Look for the add river button
    const addRiverButton = page.locator('text=川を追加');
    await expect(addRiverButton).toBeVisible();
    await addRiverButton.click();

    // Fill in river details
    await page.fill('input[placeholder*="川の名前"]', 'テスト川');
    await page.fill('textarea[placeholder*="説明"]', 'これはテスト用の川です');
    
    // Submit the form
    await page.click('button[type="submit"]');
    
    // Verify the river was added
    await expect(page.locator('text=テスト川')).toBeVisible();
  });

  test('should display rivers on the map', async ({ page }) => {
    // Wait for map to load
    await expect(page.locator('.leaflet-container')).toBeVisible();
    
    // Check if there are any markers or overlays on the map
    const mapMarkers = page.locator('.leaflet-marker-icon');
    const mapOverlays = page.locator('.leaflet-overlay-pane');
    
    // At least one of these should be present if rivers are displayed
    await expect(mapMarkers.or(mapOverlays)).toBeVisible();
  });
});