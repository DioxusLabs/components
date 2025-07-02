import { test, expect } from "@playwright/test";

test("test", async ({ page }) => {
  await page.goto("http://127.0.0.1:8080/component/?name=calendar&", { timeout: 20 * 60 * 1000 }); // Increase timeout to 20 minutes
  // Find the calendar element
  const calendar = page.locator(".calendar");
  // Find the calendar-nav-prev button
  const prevButton = calendar.locator(".calendar-nav-prev");
  // Find the calendar-nav-next button
  const nextButton = calendar.locator(".calendar-nav-next");

  // Assert the calendar is displayed
  await expect(calendar).toBeVisible();
  // Assert the current month is displayed
  const currentMonth = calendar.locator(".calendar-month-title");
  let currentMonthText = (await currentMonth.textContent()) || "";

  // Click the previous button to go to the previous month
  await prevButton.click();
  // Assert the month has changed
  let previousMonthText = await currentMonth.textContent();
  expect(previousMonthText).not.toBe(currentMonthText);

  // Click the next button to go back to the current month
  await nextButton.click();
  // Assert the month has changed back to the current month
  await expect(currentMonth).toHaveText(currentMonthText);

  // Move focus to the calendar with tab
  await page.keyboard.press("Tab");
  const focusedDay = calendar.locator(
    '.calendar-grid-cell[data-month="current"]:focus'
  );
  // Assert a day is focused
  const firstDay = focusedDay.first();
  // Get the day
  const day = await firstDay.textContent();
  const dayNumber = parseInt(day || "", 10);
  // Pressing right arrow should move focus to the next day
  await page.keyboard.press("ArrowRight");
  const nextDay = focusedDay.first();
  // Assert the next day is focused
  const nextDayNumber = parseInt((await nextDay.textContent()) || "", 10);
  expect(nextDayNumber).toBe(dayNumber + 1);
  // Pressing left arrow should move focus back to the first day
  await page.keyboard.press("ArrowLeft");
  await expect(firstDay).toContainText(day || "failure");
  // Pressing down arrow should move focus to the next week
  await page.keyboard.press("ArrowDown");
  const nextWeekDay = focusedDay.first();
  // Assert the next week day is focused
  const nextWeekDayNumber = parseInt(
    (await nextWeekDay.textContent()) || "",
    10
  );
  expect(nextWeekDayNumber).toBe(dayNumber + 7);
  // Pressing up arrow should move focus back to the first day of the month
  await page.keyboard.press("ArrowUp");
  await expect(firstDay).toContainText(day || "failure");
});
